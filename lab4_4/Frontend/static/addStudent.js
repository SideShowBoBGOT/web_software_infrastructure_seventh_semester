import axios from 'https://cdn.jsdelivr.net/npm/axios@1.3.5/+esm';

document.getElementById("addStudent").addEventListener("click", () => {
    console.log("uhm")
    axios.post("/addStudent/commit", {
        "group": document.getElementById("groups-big").value.replace("group-", ""),
        "name": document.getElementById("name").value,
        "surname": document.getElementById("surname").value
    }).then(location.reload())
})