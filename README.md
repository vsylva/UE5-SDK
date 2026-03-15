<div align="center">
  <img src="assets/demo.png" alt="" width="800px">
</div>


This is an SDK I developed using Rust. It has been in use for some time and works well.

Primarily developed for ARK ASA. It can also be adapted for other Unreal Engine games after modification.

Some code is ported from the dumper-7 SDK.

I used attribute macros to simulate C++ memory layouts, and used deref to simulate inheritance-based method calls.

For well-known reasons, you need to locate certain function addresses on their own and implement spoofcall themselves.

A small utility is also included, which can replace the offsets in the SDK with the latest ones from the dumper-7 SDK.

./offsets_replacer.exe  C:\Dumper-7\5.5.4-801548+++ARK1+Rel-1.83-ShooterGame\CppSDK  ../../src/SDK