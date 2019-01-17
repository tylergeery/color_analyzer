function b64EncodeUnicode(str) {
    // first we use encodeURIComponent to get percent-encoded UTF-8,
    // then we convert the percent encodings into raw bytes which
    // can be fed into btoa.
    return btoa(encodeURIComponent(str).replace(/%([0-9A-F]{2})/g,
        function toSolidBytes(match, p1) {
            return String.fromCharCode('0x' + p1);
    }));
}

class FormHandler {
    constructor($form) {
        this.$form = $form;

        $form.find('input')
            .on('submit change', (e) => {
                e.preventDefault();

                this.handleSubmit();
            });
    }

    gatherOptions() {
        return Promise.resolve();
    }

    gatherURL() {
        return this.$form.attr('action');
    }

    handleSubmit() {
        $('.ica-results-image-holder')
            .html(`<img src="https://media.giphy.com/media/3oEjI6SIIHBdRxXI40/giphy.gif" alt="loading image" />`);

        this.gatherOptions()
            .then((options) => {
                $.ajax(options)
                    .done((response) => {
                        this.renderImage();
                        this.handleResults(response);
                    })

                    .fail((error, first, third) => {
                        this.handleFailure();
                    });
            });
    }

    handleFailure() {
        $('.ica-results-image-holder').html(``);
        $('.ica-results-holder').html(`Failure parsing image`);
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
    gatherOptions() {
        let url = this.gatherURL();
        let imageURL = this.$form.find('input[name="url"]').val();
        let options = {
            method: 'POST',
            contentType: 'application/json',
            data: JSON.stringify({
                url: imageURL
            }),
            url
        };

        return Promise.resolve(options);
    }

    renderImage() {
        let $image = this.$form.find('input[name="url"]');
        let src = $image.val();

        $('.ica-results-image-holder')
            .html(`<img src="${src}" alt="lookup image" />`);
    }
}

class FileUploadFormHandler extends FormHandler {
    gatherOptions() {
        let url = this.gatherURL();
        let data = new FormData(this.$form[0]);
        let options = {
            method: 'POST',
            contentType: 'multipart/form-data',
            processData: false,
            contentType: false,
            data,
            url
        };

        return Promise.resolve(options);
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
