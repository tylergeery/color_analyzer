class FormHandler {
    constructor($form) {
        this.$form = $form;

        $form.find('input')
            .on('submit change', (e) => {
                e.preventDefault();

                this.handleSubmit(
                    this.gatherURL(),
                    this.gatherData(),
                    this.getContentType()
                );
            });
    }

    gatherData() {
        // TODO: overwrite with child class
    }

    gatherURL() {
        return this.$form.attr('action');
    }

    getContentType() {
        return this.$form.attr('enctype') || 'application/json';
    }

    handleSubmit(url, data, contentType) {
        $.ajax({
            method: 'POST',
            data,
            contentType,
            url
        })
            .done((response) => {
                this.handleResults(response);
            })

            .fail((error, first, third) => {
                console.log('error: ', error, first, third);
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
        return JSON.stringify({
            url: this.$form.find('input[name="url"]').val()
        });
    }
}

class FileUploadFormHandler extends FormHandler {
    gatherData() {
        let fd = new FormData();

        fd.append('image', this.$form.find('input')[0].files);

        return fd;
    }
}

new URLFormHandler($('#ica-form-url'));
new FileUploadFormHandler($('#ica-form-file'));
