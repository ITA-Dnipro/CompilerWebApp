
module.exports = function(grunt) {
    grunt.initConfig({
        jshint: {
          files: ['Gruntfile.js', 'static/js/*.js', 'tests/*.js'],
          options: {
            globals: {
              jQuery: true
            },
            asi: true,
            esnext: 6,
            reporterOutput: ""
          }
        },
        'js-test': {
            options: {
                pattern: 'tests/*.js',
                
                deps: [
                    'static/js/ace-builds/src-noconflict/ace.js',
                    'static/js/editor.js',
                    'static/js/submit_form.js'
                ]
            },
            default: {
                options: {
                    pattern: 'tests/*.js'
                }
            }
        },
        watch: {
          files: [],
          tasks: ['jshint']
        }
      });
    grunt.loadNpmTasks('grunt-contrib-jshint')
    grunt.loadNpmTasks('grunt-js-test')
    grunt.registerTask('default', ['jshint'])
    grunt.registerTask('test', ['js-test'])
}