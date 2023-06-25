#!/bin/bash

apt -y update
apt -y upgrade
apt -y install unzip
apt -y install build-essential

# Configure vim
echo 'set nocompatible
filetype on
filetype plugin on
syntax on
set relativenumber
set cursorline
set cursorcolumn
set shiftwidth=4
set tabstop=4
set expandtab
set nobackup
set scrolloff=10
set nowrap
set incsearch
set ignorecase
set smartcase
set showcmd
set showmode
set showmatch
set hlsearch
set history=1000
filetype indent on
set wildmenu
set wildmode=list:longest
set wildignore=*.docx,*.jpg,*.png,*.gif,*.pdf,*.pyc,*.exe,*.flv,*.img,*.xlsx' > $HOME/.vimrc

# Download k6
wget https://github.com/grafana/k6/releases/download/v0.44.1/k6-v0.44.1-linux-amd64.tar.gz
tar -xzf k6-v0.44.1-linux-amd64.tar.gz k6-v0.44.1-linux-amd64/k6
mv k6-v0.44.1-linux-amd64/k6 /root/k6
rm -rf k6-v0.44.1-linux-amd64
rm -r k6-v0.44.1-linux-amd64.tar.gz

export PATH=$PATH:/root/k6
