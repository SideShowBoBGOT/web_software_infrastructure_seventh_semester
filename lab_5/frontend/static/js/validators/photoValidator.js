const PhotoValidator = {
    validateFile(file) {
        if (!file) return true;

        const validTypes = ['image/jpeg', 'image/png', 'image/jpg'];
        if (!validTypes.includes(file.type)) {
            Utils.showAlert('Please select a JPEG or PNG image.');
            return false;
        }

        return true;
    }
};