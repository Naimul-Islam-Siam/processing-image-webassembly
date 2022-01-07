async function init() {
   const rustApp = null;

   try {
      rustApp = await import('../pkg/index');
   } catch (e) {
      console.error(e);
      return;
   }

   console.log(rustApp);

   const input = document.getElementById('upload');
   const fileReader = new FileReader();

   fileReader.onloadend = () => {
      let base64 = fileReader.result.replace(/^data:image\/(png|jpeg|jpg);base64,/, '');

      console.log(base64);
   };

   input.addEventListener('change', () => {
      fileReader.readAsDataURL(input.files[0]);
   });
}

init();