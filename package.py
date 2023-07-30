import os, tomllib

loaded_struct: dict = {}
with open("Cargo.toml", "rb") as f:
    loaded_struct = tomllib.load(f)

KAMIYA_VERSION = loaded_struct["package"]["version"]
KAMIYA_TARGET_PLATFORM = "linux" 
KAMIYA_TARGET_ARCHITECTURE = "x86_64"
KAMIYA_TARGET_DIR = "release"

if not os.path.exists(f"target/{KAMIYA_TARGET_DIR}/kamiya"):
    print("ERROR: run `make release` first.")
    exit(1)

print(f"INFO: making package `kamiya-{KAMIYA_VERSION}-{KAMIYA_TARGET_PLATFORM}-{KAMIYA_TARGET_ARCHITECTURE}`")
os.system(f"cp target/{KAMIYA_TARGET_DIR}/kamiya kamiya && tar -cf kamiya-{KAMIYA_VERSION}-{KAMIYA_TARGET_PLATFORM}-{KAMIYA_TARGET_ARCHITECTURE}.tar.xz kamiya && rm kamiya")
print("INFO: Done")
