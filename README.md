### An app to set Global Settings on the Moog Matriarch synthesizer

## Install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  
source $HOME/.cargo/env  

sudo apt-get install -y libasound2-dev  
sudo apt-get install -y libadwaita-1-dev  
    sudo apt-get install -y libgtk-4-dev  
    sudo apt-get install -y libgraphene-1.0-dev  
    sudo apt-get install -y libgdk-pixbuf-2.0-dev  
    sudo apt-get install -y libpango1.0-dev  
    
## To do:  
- Allow to set midi port from combo rather than just selecting moog matriarch port
- Add gtkcellrenderer hscale (slider)
- Figure out why gktcellrenderer combos do not scroll
