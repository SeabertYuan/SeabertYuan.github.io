function getRandomSize(min, max) {
    return Math.round(Math.random() * (max-min) + min);
}

var allImages = "";

for(var i = 1; i <= 4; i++){
    allImages = `<img src="/resources/gallery-images/img-${i}.jpg" alt="my pictures">`;
    $('#galleryColumn0').append(allImages);
    console.log("image"+ i + allImages);
}
for(var i = 5; i <= 8; i++){
    allImages = `<img src="/resources/gallery-images/img-${i}.jpg" alt="my pictures">`;
    $('#galleryColumn1').append(allImages);
    console.log("image"+ i + allImages);
}
for(var i = 9; i <= 11; i++){
    allImages = `<img src="/resources/gallery-images/img-${i}.jpg" alt="my pictures">`;
    $('#galleryColumn2').append(allImages);
    console.log("image"+ i + allImages);
}
