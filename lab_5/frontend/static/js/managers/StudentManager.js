class StudentManager {
    constructor() {
        this.blobUrls = new Set(); // Track blob URLs for cleanup
        this.bindEvents();
        this.initialize();
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
            await this.renderStudents(filteredStudents);
        } catch (error) {
            Utils.handleApiError(error, 'load students');
        }
    }

    async renderStudents(students) {
        const table = document.getElementById('studentsTable');
        
        // Cleanup existing blob URLs
        this.cleanupBlobUrls();
        
        // Clear the table first
        table.innerHTML = '';

        // Create and append rows one by one
        for (const student of students) {
            const row = document.createElement('tr');
            
            // Get image URL first
            let imageUrl = await API.students.getImage(student.id);
            if (imageUrl) {
                this.blobUrls.add(imageUrl); // Track the new blob URL
            } else {
                imageUrl = '/static/img/placeholder.png'; // Fallback image
            }
            
            row.innerHTML = `
                <td>${student.id}</td>
                <td>
                    <img src="${imageUrl}" 
                        alt="Student Photo" 
                        class="student-photo"
                        onerror="this.src='/static/img/placeholder.png'"
                        width="160"
                        height="120">
                </td>
                <td>${student.name}</td>
                <td>${student.surname}</td>
                <td>${student.group_id}</td>
                <td>
                    <a href="/update-student?id=${student.id}" class="edit-btn">Edit</a>
                    <button onclick="studentManager.deleteStudent(${student.id})" class="delete-btn">Delete</button>
                </td>
            `;
            
            table.appendChild(row);
        }
    }

    cleanupBlobUrls() {
        // Revoke all existing blob URLs
        for (const url of this.blobUrls) {
            URL.revokeObjectURL(url);
        }
        this.blobUrls.clear();
    }

    // Make sure to cleanup when the component is destroyed
    destroy() {
        this.cleanupBlobUrls();
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