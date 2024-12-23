class StudentUpdater {
    constructor() {
        this.studentId = new URLSearchParams(window.location.search).get('id');
        this.initialize();
    }

    async initialize() {
        if (!this.studentId) {
            Utils.showAlert('No student ID provided');
            window.location.href = '/students';
            return;
        }

        this.bindEvents();
        await this.loadGroups();
        await this.loadStudentData();
    }

    bindEvents() {
        document.getElementById('updateStudentForm')
            .addEventListener('submit', this.handleUpdateStudent.bind(this));
        document.getElementById('studentPhoto')
            .addEventListener('change', (e) => PhotoValidator.validateFile(e.target.files[0]));
    }

    async loadGroups() {
        try {
            const groups = await API.groups.getAll();
            Utils.populateGroupSelect(document.getElementById('studentGroup'), groups);
        } catch (error) {
            Utils.handleApiError(error, 'load groups');
        }
    }

    async loadStudentData() {
        try {
            const student = await API.students.get(this.studentId);
            this.populateStudentForm(student);
        } catch (error) {
            Utils.handleApiError(error, 'load student data');
        }
    }

    populateStudentForm(student) {
        document.getElementById('studentId').value = student.id;
        document.getElementById('studentName').value = student.name;
        document.getElementById('studentSurname').value = student.surname;
        document.getElementById('studentGroup').value = student.group_id;

        const photoElement = document.getElementById('currentPhoto');
        photoElement.src = API.students.getImageUrl(student.id);
        photoElement.onerror = () => {
            photoElement.src = '/static/img/placeholder.png';
        };
    }

    async handleUpdateStudent(event) {
        event.preventDefault();
        const formData = new FormData(event.target);
        const photoFile = formData.get('studentPhoto');

        if (photoFile && !PhotoValidator.validateFile(photoFile)) {
            return;
        }

        try {
            await API.students.update(this.studentId, formData);
            Utils.showAlert('Student updated successfully');
            window.location.href = '/students';
        } catch (error) {
            Utils.handleApiError(error, 'update student');
        }
    }
}