function submitForm(event) {
    event.preventDefault()
    let error_div_id = 'stderr-wrapper'
    let output_div_id = 'stdout-wrapper'
    let run_output_div_id = 'run-stdout-wrapper'
    let run_stderr_div_id = 'run-stderr-wrapper'
    let run_success_div_id = 'run-success'
    let data_to_send = parseDataToSend()
    let request = new Request(
        event.target.action,
        {
            method: "POST",
            body: JSON.stringify(data_to_send),
            headers: {
                "Content-Type" : "application/json",
                "execute" : document.getElementById("if-execute").checked
            }
        }
    )

    fetch (request)
        .then (
            (response) => {
                    return response.json()
            }
        ).then (
            (data_json) => {
                buildStdoutBlock(output_div_id, data_json.stdout)
                buildStderrBlock(error_div_id, data_json.stderr)
                if ('runner_output' in data_json) {
                    buildRunSuccessBlock(run_success_div_id, data_json)
                    buildStdoutBlock(run_output_div_id, data_json.runner_output.stdout)
                    buildStdoutBlock(run_stderr_div_id, data_json.runner_output.stderr)
                } else {
                    buildRunSuccessBlock(run_success_div_id, null)
                    buildStdoutBlock(run_output_div_id, '')
                    buildStderrBlock(run_stderr_div_id, '')
                }
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

function buildRunSuccessBlock(div_id, data_json) {
    let div_block = document.getElementById(div_id)
    if (data_json === null){
        div_block.innerHTML = ''

        return
    }
    let stderr = data_json.runner_output.stderr
    let exit_code = data_json.runner_output.exit_code
    if (exit_code !== null && !stderr) {
        div_block.innerHTML =
            '<div id="execution-finished" class=="alert alert-info">' +
            "Execution finished with exit code: " + exit_code +
            '</div>'
    } else {
        div_block.innerHTML = ''
    }
}

document
    .getElementById("source-code-form")
    .addEventListener("submit", submitForm)

  