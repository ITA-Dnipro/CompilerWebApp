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
    fetch (request)
        .then (
            (response) => {
                    return response.json()
            }
        ).then (
            (data_json) => {
                console.log(data_json)
                let div_error_message = document.getElementById("stderr-wrapper")
                if (data_json["stderr"]) {
                    div_error_message.innerHTML = '<div id="stderr" class="alert alert-danger">'
                        + data_json["stderr"].toString()
                    + '</div>'
                }
            }
        )
                
}                




document
    .getElementById("source-code-form")
    .addEventListener("submit", submitForm)

  