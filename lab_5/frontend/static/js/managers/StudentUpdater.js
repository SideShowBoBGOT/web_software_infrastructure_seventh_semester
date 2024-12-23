class StudentUpdater {
    constructor() {
        this.studentId = new URLSearchParams(window.location.search).get('id');
        this.initialize();
        this.cleanup = null;
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
            await this.populateStudentForm(student);
        } catch (error) {
            Utils.handleApiError(error, 'load student data');
        }
    }

    async populateStudentForm(student) {
        document.getElementById('studentId').value = student.id;
        document.getElementById('studentName').value = student.name;
        document.getElementById('studentSurname').value = student.surname;
        document.getElementById('studentGroup').value = student.group_id;
    
        // Clean up previous blob URL if it exists
        if (this.cleanup) {
            this.cleanup();
        }

        // Handle photo display
        const photoElement = document.getElementById('currentPhoto');
        if (student.id) {
            try {
                const imageUrl = await API.students.getImage(student.id);
                if (imageUrl) {
                    photoElement.src = imageUrl;
                    // Store cleanup function
                    this.cleanup = () => URL.revokeObjectURL(imageUrl);
                } else {
                    photoElement.src = '/static/img/placeholder.png';
                }
            } catch (error) {
                console.error('Error loading student image:', error);
                photoElement.src = '/static/img/placeholder.png';
            }
        } else {
            photoElement.src = '/static/img/placeholder.png';
        }
    }

    async handleUpdateStudent(event) {
        event.preventDefault();
        const formData = new FormData(event.target);
        const photoFile = formData.get('studentPhoto');
    
        // Only validate if a photo was provided and has size
        if (photoFile && photoFile.size > 0) {
            if (!PhotoValidator.validateFile(photoFile)) {
                return;
            }
        } else {
            // If no new photo was selected, remove it from formData
            formData.delete('studentPhoto');
        }
    
        try {
            await API.students.update(this.studentId, formData);
            Utils.showAlert('Student updated successfully');
            
            // Clean up blob URL before navigation
            if (this.cleanup) {
                this.cleanup();
            }
            
            window.location.href = '/students';
        } catch (error) {
            Utils.handleApiError(error, 'update student');
        }
    }

    // Cleanup method to be called when component is destroyed
    destroy() {
        if (this.cleanup) {
            this.cleanup();
        }
    }
}