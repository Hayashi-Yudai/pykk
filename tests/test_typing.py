import ast
from importlib.metadata import distribution
from pathlib import Path

import pykk


def _installed_package_dir() -> Path:
    """Return the directory that contains the installed extension package."""

    return Path(pykk.__file__).resolve().parent


def _public_stub() -> tuple[Path, ast.Module]:
    package_dir = _installed_package_dir()
    stubs = sorted(package_dir.glob("*.pyi"))
    assert stubs, f"no type stub was installed in {package_dir}"

    stub = next((path for path in stubs if path.stem == "pykk"), stubs[0])
    return stub, ast.parse(stub.read_text(encoding="utf-8"))


def test_public_stub_describes_transform_functions_and_version():
    stub, module = _public_stub()
    functions = {
        node.name: node
        for node in module.body
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef))
    }

    for name in ("real2imag", "imag2real"):
        function = functions.get(name)
        assert function is not None, f"{name} is missing from {stub}"
        assert len(function.args.posonlyargs) == 2
        assert [arg.arg for arg in function.args.posonlyargs] == ["x", "y"]
        assert all(
            ast.unparse(arg.annotation) == "ArrayLike"
            for arg in function.args.posonlyargs
        )
        assert ast.unparse(function.returns) == "NDArray[np.float64]"

    version = next(
        (
            node
            for node in module.body
            if isinstance(node, ast.AnnAssign)
            and isinstance(node.target, ast.Name)
            and node.target.id == "__version__"
        ),
        None,
    )
    assert version is not None, f"__version__ is missing from {stub}"
    assert ast.unparse(version.annotation) == "str"


def test_type_marker_is_in_installed_distribution():
    package_dir = _installed_package_dir()
    marker = package_dir / "py.typed"

    assert marker.is_file(), f"py.typed is missing from {package_dir}"

    files = distribution("pykk").files or ()
    assert any(path.as_posix().endswith("/py.typed") for path in files)
