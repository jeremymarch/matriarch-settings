### An app to set Global Settings on the Moog Matriarch synthesizer

Modeled after: https://mreid.github.io/matriarch-editor/index.html  
See also: https://forum.moogmusic.com/viewtopic.php?t=34523  


## Install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  
source $HOME/.cargo/env  

sudo apt-get install -y libasound2-dev  
sudo apt-get install -y libadwaita-1-dev  
    
## To do:  
- Allow to set midi port from combo rather than just selecting moog matriarch port
- Add gtkcellrenderer hscale (slider)
