# CompilerWebApp

CompilerWebApp is a web server that allows users to study their code online without any burden of environment setup. 

## Major Features
* Customers can input, edit, view the code on the main product’s page
* Customers can choose compilations options
* Customers can submit their code and receive the results of the compilation

## User's guide
### How to launch the server
The app requires the _nightly_ toolchain. It can be run with `cargo +nightly run` if the toolchain is installed,
or just with `cargo run` if it also set as the default toolchain.

Also the server requires a config file `BackendConfig.yaml`, where you can set required configurations:
- `sessions_data_dir` - path to a folder where sessions data folders will be stored;
- `sessions_data_file` - path to a file, where sessions tracker data will be serialized to and deserialized from;
All time-based fields store time values in milliseconds.
----
- `session_life_duration` - amount of time a session can be active for;
- `sessions_cleanup_interval` - interval of time that server waits before cleaning up expired sessions from the tracker and `sessions_data_dir`;
- `sessions_save_interval` - interval of time that server waits before saving sessions tracker to the `sessions_data_file`;
----
- `lang_extensions` - a map of languages, as strings, and their respective source code files extensions, as strings.

### Anonymous user sessions
The server keeps track of anonymous user sessions, which means that users don't have to sign in to use the service. 
User files are saved on the server for as long as the session is alive.

### Endpoints
- `GET: "/"`:
Returns `index.html`. If the session has already had source code submitted it will be rendered on the page.

- `POST: "/submit"`:
headers:
```
Content-Type: "application/json",
execute: bool
```
`execute` header specifies whether the code should also be executed.
The code will only be executed if the header's value can be parsed to `true` and the code can be compiled.
This header may be omitted, then its value will be interpreted as `false`.

body: 
```
{
    "code": string,
    "lang": string,
    "options": string
}
```

Returns result of the compilation:

headers:
```
Content-Type: "application/json"
```
body: 
```
{
    "status code": int,
    "stdout": string,
    "stderr": string,
    "runner_output":
    {
        "stdout": string,
        "stderr": string,
        "exit_code": int
    }
}
```

`runner_output` field is only present if the `execute` flag was true and the code could be compiled.
