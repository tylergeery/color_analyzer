class FormHandler {
    constructor($form) {
        this.$form = $form;

        $form.find('input')
            .on('submit change', (e) => {
                e.preventDefault();

                this.handleSubmit(
                    this.gatherURL()
                );
            });
    }

    gatherData() {
        return Promise.resolve();
    }

    gatherURL() {
        return this.$form.attr('action');
    }

    handleSubmit(url) {
        this.gatherData()
            .then((data) => {
                $.ajax({
                    method: 'POST',
                    contentType: 'application/json',
                    data,
                    url
                })
                    .done((response) => {
                        this.handleResults(response);
                    })

                    .fail((error, first, third) => {
                        console.log('error: ', error, first, third);
                    });
            });
    }

    handleResults(response) {
        let predictions = JSON.parse(response);
        let $resultsHolder = $('.ica-results-holder');

        $resultsHolder.html("");

        predictions.map((p) => {
            let html = `<div style='background-color: ${p.name}'>${p.name} (${p.score})</div>`;

            $resultsHolder.append(html);
        });
    }
}

class URLFormHandler extends FormHandler {
    gatherData() {
        return Promise.resolve(JSON.stringify({
            url: this.$form.find('input[name="url"]').val()
        }));
    }
}

class FileUploadFormHandler extends FormHandler {
    gatherData() {
        return new Promise((resolve, reject) => {
            var reader = new FileReader();
            reader.onload = (readerEvt) => {
                console.log('contents:', readerEvt.target.result);
                let data = btoa(readerEvt.target.result);

                resolve(JSON.stringify({ file: data }));
            };
            reader.readAsBinaryString(this.$form.find('input')[0].files[0]);
        });
    }
}

new URLFormHandler($('#ica-form-url'));
new FileUploadFormHandler($('#ica-form-file'));
