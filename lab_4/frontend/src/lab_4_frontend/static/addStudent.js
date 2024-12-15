import axios from 'https://cdn.jsdelivr.net/npm/axios@1.3.5/+esm';

document.getElementById("addStudent").addEventListener("click", () => {
    console.log("uhm")
    const group = document.getElementById("groups-big").value.replace("group-", "")
    const stName = document.getElementById("name").value
    const stSurname = document.getElementById("surname").value
    axios.post("/addStudent/commit/", {
        "group": group,
        "name": stName,
        "surname": stSurname 
    })
})