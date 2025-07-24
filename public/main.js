console.log('test');

async function init() {
    let rustApp = null;

    try {
        rustApp = await import('../pkg/')
    } catch(e) {
        console.error(e)
        return;
    }

    console.log(rustApp)

    const input = document.getElementById('upload');
    const fileReader = new FileReader();  

    fileReader.onload = (event) => {
        const base64 = fileReader.result.replace(
            /^data:.+;base64,/,
            ''
        );
        // console.log(input.files[0]);
        // console.log(base64);
        let img_data_url = rustApp.grayscale(base64);
        // console.log(img_data_url);
        document.getElementById('new-image').setAttribute('src', img_data_url);
    }
    input.addEventListener('change', (event) => {
        fileReader.readAsDataURL(input.files[0])});
}

init();