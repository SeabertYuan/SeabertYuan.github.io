function getRandomSize(min, max) {
    return Math.round(Math.random() * (max-min) + min);
}

var allImages = "";

for(var i = 1; i <= 5; i++){
    allImages = `<img src="/resources/gallery-images/img-${i}.jpg" alt="my pictures">`;
    $('#gallery').append(allImages);
    console.log("image"+ i + allImages);
}