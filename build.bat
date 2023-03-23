@echo off
cargo clean
cargo +nightly b -Z build-std=core,alloc,panic_abort -Z build-std-features=panic_immediate_abort --target i686-pc-windows-msvc --release
"C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x86\mt.exe" -manifest "quark.exe.manifest" -outputresource:".\target\i686-pc-windows-msvc\release\quark.exe";#1
powershell -c Set-AuthenticodeSignature .\target\i686-pc-windows-msvc\release\quark.exe -Certificate (Get-ChildItem Cert:\CurrentUser\My -CodeSigningCert)
copy .\target\i686-pc-windows-msvc\release\quark.exe .