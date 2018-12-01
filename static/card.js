$(function(){

    $('.show').on('click',function(){
        var closest_reveal = $(this)[0].closest('.card').children[2];
        $(closest_reveal).slideToggle('slow');
    });
    
    $('.card-reveal .close').on('click',function(){
        var closest_reveal = $(this)[0].parentElement;
        $(closest_reveal).slideToggle('slow');
    });
});