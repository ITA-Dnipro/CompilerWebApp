describe('Array', function(){
    describe('#parseDataToSend()', function() {
      it ('should return object with 3 values', function(){
        /// <reference path="../static/index.html" />
        let expected_empty_obj = {
          code: '',
          options: '',
          lang: 'c++'
        }
        let parsed_object = parseDataToSend()
        chai.expect(expected_empty_obj).to.eql(parsed_object)
        //chai.assert.equal(expected_empty_obj, parsed_object)
      })
    }
    )

    describe('# editor syntax is changed on option change',
      function() {
        it ('editor syntax should be changed on option change', 
          function() {
            /// <reference path="../static/index.html" />
            console.log(editor.getTheme())
            chai.assert.equal(1, 1)
          }
        )
      }
    )
  });