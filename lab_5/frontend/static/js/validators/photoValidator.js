const PhotoValidator = {
    validateFile(file) {
        // If no file is provided, validation passes (photo is optional)
        if (!file || file.size === 0) return true;

        const validTypes = ['image/jpeg', 'image/png', 'image/jpg'];
        if (!validTypes.includes(file.type)) {
            Utils.showAlert('Please select a JPEG or PNG image, or leave the field empty.');
            return false;
        }

        return true;
    }
};