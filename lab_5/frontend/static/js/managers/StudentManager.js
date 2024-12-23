class StudentManager {
    constructor() {
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
            const students = await API.students.getAll(groupId);
            this.renderStudents(students);
        } catch (error) {
            Utils.handleApiError(error, 'load students');
        }
    }

    renderStudents(students) {
        const table = document.getElementById('studentsTable');
        table.innerHTML = students.map(student => `
            <tr>
                <td>${student.id}</td>
                <td>
                    <img src="${API.students.getImageUrl(student.id)}" 
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
            </tr>
        `).join('');
    }

    async handleAddStudent(event) {
        event.preventDefault();
        const formData = new FormData(event.target);
        const photoFile = formData.get('studentPhoto');

        if (!PhotoValidator.validateFile(photoFile)) {
            return;
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