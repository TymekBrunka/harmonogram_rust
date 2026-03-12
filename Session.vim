let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/Documents/kody/harmonogram_rust
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +1 ~/Documents/kody/harmonogram-rust/Cargo.toml
badd +1 ~/Documents/kody/harmonogram-rust/src/iced_minimal_example.rs
badd +1 ~/Documents/kody/harmonogram-rust/src/iced_panes.rs
badd +37 ~/Documents/kody/harmonogram-rust/src/main.rs
badd +13 ~/Documents/kody/harmonogram_rust/Cargo.toml
badd +38 ~/Documents/kody/harmonogram_rust/src/main.rs
badd +32 ~/Documents/kody/harmonogram_rust/src/minimal.rs
badd +237 ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/iced_widget-0.14.2/src/helpers.rs
badd +599 ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/iced_widget-0.14.2/src/button.rs
badd +140 ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/iced_core-0.14.0/src/theme.rs
badd +398 ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/iced_core-0.14.0/src/theme/palette.rs
badd +2 ~/Documents/kody/harmonogram_rust/src/widgets.rs
argglobal
%argdel
tabnew +setlocal\ bufhidden=wipe
tabrewind
edit ~/Documents/kody/harmonogram_rust/src/widgets.rs
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
split
1wincmd k
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe '1resize ' . ((&lines * 41 + 27) / 55)
exe '2resize ' . ((&lines * 10 + 27) / 55)
argglobal
balt ~/Documents/kody/harmonogram_rust/src/main.rs
setlocal foldmethod=manual
setlocal foldexpr=0
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldenable
silent! normal! zE
let &fdl = &fdl
let s:l = 2 - ((1 * winheight(0) + 20) / 41)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 2
normal! 015|
wincmd w
argglobal
enew
balt ~/Documents/kody/harmonogram_rust/src/widgets.rs
setlocal foldmethod=manual
setlocal foldexpr=0
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=99
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal nofoldenable
wincmd w
exe '1resize ' . ((&lines * 41 + 27) / 55)
exe '2resize ' . ((&lines * 10 + 27) / 55)
tabnext
edit ~/Documents/kody/harmonogram_rust/src/main.rs
argglobal
balt ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/iced_widget-0.14.2/src/helpers.rs
setlocal foldmethod=manual
setlocal foldexpr=0
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldenable
silent! normal! zE
let &fdl = &fdl
let s:l = 38 - ((37 * winheight(0) + 26) / 52)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 38
normal! 018|
tabnext 2
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
