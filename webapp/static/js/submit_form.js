function submitForm(event) {
    event.preventDefault()
    var form_data = new FormData(event.target)
    let obj = {}
    obj["code"] = editor.getValue()
    obj["lang"] = document.getElementById("lang-input").value
    obj["options"] = document.getElementById("options-input").value
    let request = new Request(
        event.target.action,
        {
            method: "POST",
            body: JSON.stringify(obj),
            headers: {"Content-Type" : "application/json"}
        }
    )
    fetch (request).then(
        (response) => console.log(response)
    )
}

document
    .getElementById("source-code-form")
    .addEventListener("submit", submitForm)

  