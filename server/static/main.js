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
        $('.ica-results-image-holder')
            .html(`<img src="https://media.giphy.com/media/3oEjI6SIIHBdRxXI40/giphy.gif" alt="loading image" />`);

        this.gatherData()
            .then((data) => {
                $.ajax({
                    method: 'POST',
                    contentType: 'application/json',
                    data,
                    url
                })
                    .done((response) => {
                        this.renderImage();
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

        $resultsHolder.html('');

        predictions.map((p) => {
            let score = Math.round(p.score * 100000) / 100000;
            let subImage = `<div><div style='background-color: ${p.hex}'></div></div>`;
            let contents = `<div>${score}<br />${p.name}</div>`;
            let html = `<div class="ica-result">${subImage}${contents}</div>`;

            $resultsHolder.append(html);
        });
    }

    renderImage() {}
}

class URLFormHandler extends FormHandler {
    gatherData() {
        return Promise.resolve(JSON.stringify({
            url: this.$form.find('input[name="url"]').val()
        }));
    }

    renderImage() {
        let $image = this.$form.find('input[name="url"]');
        let src = $image.val();

        $('.ica-results-image-holder')
            .html(`<img src="${src}" alt="lookup image" />`);
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

    renderImage() {
        var reader = new FileReader();
        reader.onload = () => {
            $('.ica-results-image-holder')
                .html(`<img src="${reader.result}" alt="lookup image" />`);
        };
        reader.readAsDataURL(this.$form.find('input')[0].files[0]);
    }
}

new URLFormHandler($('#ica-form-url'));
new FileUploadFormHandler($('#ica-form-file'));
