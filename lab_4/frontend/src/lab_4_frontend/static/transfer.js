import axios from 'https://cdn.jsdelivr.net/npm/axios@1.3.5/+esm';

const groupSelect = document.getElementById("groups");
const confirmButton = document.getElementById("confirmButton");

groupSelect.addEventListener("change", () => {
    const group = groupSelect.value;
    confirmButton.disabled = !group; // Enable button only if a group is selected
});

confirmButton.addEventListener("click", () => {
    const group = groupSelect.value;
    const groupId = group.replace("group-", "");
    const studentId = new URLSearchParams(document.location.search).get("studentId");

    axios.put("/transfer/changeGroup/", {
        "studentId": studentId,
        "groupId": groupId
    }).then(response => {
        alert("Переведення успішне!");
    }).catch(error => {
        alert("Сталася помилка, спробуйте ще раз.");
    });
});
