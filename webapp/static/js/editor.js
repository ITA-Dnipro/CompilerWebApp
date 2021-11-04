var editor = ace.edit("editor")
editor.setTheme("ace/theme/chrome")
editor.session.setMode("ace/mode/c_cpp")
editor.setValue('')


var reader = new FileReader()
reader.onload = function(evt) {
    editor.setValue(evt.target.result)
}   
function handleLoadedFile(value) {
    var file = document.getElementById("file_input").files[0]
    if (file) {
        reader.readAsText(file, "UTF-8")
    }
}

