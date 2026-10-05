from pathlib import Path
from shutil import rmtree

from PIL import Image

package = Path(__file__).parent / "package"
assets = package / "Assets"

light = Image.open(package / "icon.png").convert("RGBA")
dark = Image.open(package / "icon-dark.png").convert("RGBA")

rmtree(assets, ignore_errors=True)
assets.mkdir()


def save(image, size, name):
    image.resize((size, size), Image.Resampling.LANCZOS).save(assets / name)


# Logos the manifest requires. The app is hidden from Start, so only Settings shows them
save(light, 50, "StoreLogo.png")
save(light, 150, "Square150x150Logo.png")
save(light, 44, "Square44x44Logo.png")

# Explorer menu icons, named after the theme they're for. GetIcon in command.rs picks one
menu_sizes = [(size, size) for size in (16, 20, 24, 32, 40, 48, 64, 256)]
dark.save(assets / "menu-light.ico", sizes=menu_sizes)
light.save(assets / "menu-dark.ico", sizes=menu_sizes)

print(f"Generated {len(list(assets.iterdir()))} files in {assets}")
