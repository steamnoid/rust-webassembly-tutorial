console.log('test');

function init() {
    const input = document.getElementById('upload');
    const fileReader = new FileReader();  

    fileReader.onload = (event) => {
        const base64 = fileReader.result.replace(
            /^data:.+;base64,/,
            ''
        );
        console.log(input.files[0]);
        console.log(base64);
    }
    input.addEventListener('change', (event) => {
        fileReader.readAsDataURL(input.files[0])});
}

init();