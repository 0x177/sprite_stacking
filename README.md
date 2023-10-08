# sprite_stacking
2.5D sprite stacking in macroquad

# getting a blocky retro look
many graphics crates allow you to draw to a surface/buffer/canvas or whatever it is called then blit 
the canavs onto the screen (like [the pixels crate](https://www.github.com/parasyte/pixels)). you can make the buffer very 
small (like 1/5 * screen_size) then upscale it to fit the screen before blitting. this gives the pixelated, blocky look. a commom mistake when doing this is not setting the upscaling
algorithm for the buffer to 'nearest' before upscaling. this is a mistake because the default upscaling algorithm in many graphics crates are not good for pixel art. this causes blurring and artifacts
