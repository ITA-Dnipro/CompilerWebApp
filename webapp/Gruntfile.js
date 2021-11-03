module.exports = function(grunt) {
    grunt.initConfig({
        jshint: {
          files: ['Gruntfile.js', 'static/js/*.js'],
          options: {
            globals: {
              jQuery: true
            },
            asi: true,
            esversion: 6
          }
        },
        watch: {
          files: [],
          tasks: ['jshint']
        }
      });
    grunt.loadNpmTasks('grunt-contrib-jshint')
    grunt.registerTask('default', ['jshint'])
}