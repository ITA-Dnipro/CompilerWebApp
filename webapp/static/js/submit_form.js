function submitForm(event) {
    event.preventDefault()
    let error_div_id = 'stderr-wrapper'
    let output_div_id = 'stdout-wrapper'
    let data_to_send = parseDataToSend()
    let request = new Request(
        event.target.action,
        {
            method: "POST",
            body: JSON.stringify(data_to_send),
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
                buildStdoutBlock(output_div_id, data_json.stdout)
                buildStderrBlock(error_div_id, data_json.stderr)
            }
        )       
}               

function parseDataToSend() {
    return {
        code: editor.getValue(),
        lang: document.getElementById("lang-input").value,
        options: document.getElementById("options-input").value
    }
}

function buildStdoutBlock(output_div_id, output_message) {
    let div_output_message = document.getElementById(output_div_id)
    if (output_message) {
        div_output_message.innerHTML = 
            '<div id="stdout" class="alert alert-secondary">' + 
            output_message.toString().replace(/\n/g, '<br />') + 
            '</div>'
    } else {
        div_output_message.innerHTML = ''
    }
}

function buildStderrBlock(error_div_id, error_message) {
    let div_error_message = document.getElementById(error_div_id)
    if (error_message) {
        div_error_message.innerHTML = 
            '<div id="stderr" class="alert alert-danger">' + 
            error_message.toString().replace(/\n/g, '<br />') + 
            '</div>'
    } else {
        div_error_message.innerHTML = ''
    }
}




document
    .getElementById("source-code-form")
    .addEventListener("submit", submitForm)

  