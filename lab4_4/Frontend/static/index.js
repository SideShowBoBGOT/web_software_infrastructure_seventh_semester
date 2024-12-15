import axios from 'https://cdn.jsdelivr.net/npm/axios@1.3.5/+esm';

document.getElementById("groups").addEventListener("change", filterElementsByGroup)
function filterElementsByGroup() {
    const group = document.getElementById("groups").value
    const students = document.getElementById("content-list").getElementsByTagName("tr")
    Array.from(students).forEach(element => {
        if (element.classList.contains(group)) element.hidden = false
        else element.hidden = true
    });
}
filterElementsByGroup()

Array.from(document.getElementsByClassName("deleteStudent")).forEach(element => {
    element.addEventListener("click", () => {
        axios.post("/deleteStudent/commit", {
            "id": element.id
        }).then(location.reload())
    })
})