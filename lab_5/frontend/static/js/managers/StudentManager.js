class StudentManager {
    constructor() {
        this.bindEvents();
        this.initialize();
        this.cleanup = null;
    }

    bindEvents() {
        document.getElementById('addStudentForm')
            .addEventListener('submit', this.handleAddStudent.bind(this));
        document.getElementById('filterGroup')
            .addEventListener('change', this.handleFilterChange.bind(this));
        document.getElementById('studentPhoto')
            .addEventListener('change', (e) => PhotoValidator.validateFile(e.target.files[0]));
    }

    async initialize() {
        await this.loadGroups();
        await this.loadStudents();
    }

    async loadGroups() {
        try {
            const groups = await API.groups.getAll();
            Utils.populateGroupSelect(document.getElementById('studentGroup'), groups);
            Utils.populateGroupSelect(document.getElementById('filterGroup'), groups);
        } catch (error) {
            Utils.handleApiError(error, 'load groups');
        }
    }

    async loadStudents(groupId = null) {
        try {
            const students = await API.students.getAll();
            let filteredStudents = students;

            if (groupId && groupId !== 'all') {
                filteredStudents = students.filter(student => 
                    student.group_id.toString() === groupId.toString()
                );
            }

            // Clean up previous blob URLs if they exist
            if (this.cleanup) {
                this.cleanup();
            }
            // Store new cleanup function
            this.cleanup = await this.renderStudents(filteredStudents);
        } catch (error) {
            Utils.handleApiError(error, 'load students');
        }
    }

    async renderStudents(students) {
        const table = document.getElementById('studentsTable');
        
        // Clear the table first
        table.innerHTML = '';

        // Create and append rows one by one
        for (const student of students) {
            const row = document.createElement('tr');
            
            // Get image URL first
            const imageUrl = await API.students.getImage(student.id) || '/static/img/placeholder.png';
            
            row.innerHTML = `
                <td>${student.id}</td>
                <td>
                    <img src="${imageUrl}" 
                        alt="Student Photo" 
                        width="160"
                        height="120"
                        onerror="this.src='/static/img/placeholder.png'">
                </td>
                <td>${student.name}</td>
                <td>${student.surname}</td>
                <td>${student.group_id}</td>
                <td>
                    <a href="/update-student?id=${student.id}">Edit</a>
                    <button onclick="studentManager.deleteStudent(${student.id})">Delete</button>
                </td>
            `;
            
            table.appendChild(row);
        }

        // Clean up blob URLs when they're no longer needed
        return () => {
            const images = table.getElementsByTagName('img');
            for (const img of images) {
                if (img.src.startsWith('blob:')) {
                    URL.revokeObjectURL(img.src);
                }
            }
        };
    }

    async handleAddStudent(event) {
        event.preventDefault();
        const formData = new FormData(event.target);
        const photoFile = formData.get('studentPhoto');
    
        // Only validate if a photo was provided
        if (photoFile.size > 0 && !PhotoValidator.validateFile(photoFile)) {
            return;
        }
    
        // If no photo was selected, remove it from formData to avoid sending empty file
        if (!photoFile.size) {
            formData.delete('studentPhoto');
        }
    
        try {
            await API.students.create(formData);
            Utils.showAlert('Student added successfully');
            event.target.reset();
            this.loadStudents();
        } catch (error) {
            Utils.handleApiError(error, 'add student');
        }
    }

    async deleteStudent(id) {
        if (await Utils.confirmDelete('student')) {
            try {
                await API.students.delete(id);
                Utils.showAlert('Student deleted successfully');
                this.loadStudents();
            } catch (error) {
                Utils.handleApiError(error, 'delete student');
            }
        }
    }

    async handleFilterChange(event) {
        await this.loadStudents(event.target.value);
    }
}