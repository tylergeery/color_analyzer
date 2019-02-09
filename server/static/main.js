let selectedIndex = 0;

class FormHandler {
    constructor($form) {
        this.$form = $form;
        this.results = [];

        $form
            .on('submit', (e) => {
                e.preventDefault();

                this.handleSubmit();
            });

        $form.find('input')
            .on('change', (e) => {
                this.handleSubmit();
            });
    }

    gatherOptions() {
        return Promise.resolve();
    }

    gatherURL() {
        return this.$form.attr('action');
    }

    getAnalyzePreference() {
        return $('select[name="analyze_preference"]').val();
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
        this.results = JSON.parse(response);
        let $tabsHolder = $('.ica-results-tabs-holder');

        $tabsHolder.html(
            [
                [`Pixel`, 0],
                [`Image Center`, 1],
                [`Cluster`, 2]
            ].map(tab => {
                return `<div class="ica-tab-button-holder" data-indy=${tab[1]}>
                    <button type="button">${tab[0]}</button>
                </div>`;
            }).join('')
        );

        let self = this;
        this.renderResults(this.results[selectedIndex]);
        $(`.ica-tab-button-holder`).on('click', function () {
            selectedIndex = $(this).data('indy');
            self.renderResults(self.results[selectedIndex]);
        });
    }

    renderImage() {}

    renderResults(predictions) {
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
}

class URLFormHandler extends FormHandler {
    gatherOptions() {
        let url = this.gatherURL();
        let imageURL = this.$form.find('input[name="url"]').val();
        let options = {
            method: 'POST',
            contentType: 'application/json',
            data: JSON.stringify({
                url: imageURL,
                preference: this.getAnalyzePreference()
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

        data.append('preference', this.getAnalyzePreference());

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
