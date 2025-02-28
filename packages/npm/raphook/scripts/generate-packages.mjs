import * as fs from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { format } from "node:util";

const PACKAGES_ROOT = resolve(fileURLToPath(import.meta.url), "../../../..");
// GitHub ActionsのWorkSpaceが`/home/runner/work/raphook/raphook`という名前になっているため
const REPO_ROOT = resolve(PACKAGES_ROOT, "../../raphook");
const MANIFEST_PATH = resolve(PACKAGES_ROOT, "npm", "raphook", "package.json");

const rootManifest = JSON.parse(
  fs.readFileSync(MANIFEST_PATH).toString("utf-8")
);

function getName(platform, arch, prefix = "raphook") {
  return format(`${prefix}-${platform}`, arch);
}

function copyBinaryToNativePackage(platform, arch) {
  const os = platform.split("-")[0];
  const buildName = getName(platform, arch);
  const packageRoot = resolve(PACKAGES_ROOT, "npm", buildName);
  const packageName = buildName;

  const binDir = resolve(packageRoot, "bin");
  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }

  const { version, license, repository, engines } = rootManifest;

  const manifest = JSON.stringify(
    {
      name: packageName,
      version,
      description: `The ${
        os === "darwin" ? "macOS" : os === "win32" ? "Windows" : "Linux"
      } ${
        arch === "arm64" ? "ARM 64-bit" : "x64"
      } binary for raphook, git hooks manager.`,
      preferUnplugged: false,
      license,
      repository: {
        ...repository,
        directory: `packages/npm/${packageName}`,
      },
      os: [os],
      cpu: [arch],
      libc:
        os === "linux"
          ? packageName.endsWith("musl")
            ? ["musl"]
            : ["glibc"]
          : undefined,
    },
    null,
    2
  );

  const manifestPath = resolve(packageRoot, "package.json");
  console.log(`Update manifest ${manifestPath}`);
  fs.writeFileSync(manifestPath, manifest);

  const ext = os === "win32" ? ".exe" : "";
  const binarySource = resolve(
    REPO_ROOT,
    "dist",
    `${getName(platform, arch, "raphook")}${ext}`
  );
  const binaryTarget = resolve(binDir, `raphook${ext}`);

  if (!fs.existsSync(binarySource)) {
    console.error(
      `Source for binary for ${buildName} not found at: ${binarySource}`
    );
    process.exit(1);
  }

  fs.copyFileSync(binarySource, binaryTarget);
  fs.chmodSync(binaryTarget, 0o755);
}

function writeManifest() {
  const manifestPath = resolve(PACKAGES_ROOT, "npm", "raphook", "package.json");

  const manifestData = JSON.parse(
    fs.readFileSync(manifestPath).toString("utf-8")
  );

  const nativePackages = PLATFORMS.flatMap((platform) =>
    ARCHITECTURES.map((arch) => [
      `${getName(platform, arch)}`,
      rootManifest.version,
    ])
  );

  manifestData.version = rootManifest.version;
  manifestData.optionalDependencies = Object.fromEntries(nativePackages);

  manifestData.os = [...new Set(PLATFORMS.map((p) => p.split("-")[0]))];
  manifestData.cpu = ARCHITECTURES;
  console.log("manifestData: ", manifestData);

  console.log(`Update manifest ${manifestPath}`);
  const content = JSON.stringify(manifestData, null, 2);
  fs.writeFileSync(manifestPath, content);
}

// const PLATFORMS = ["win32-%s", "darwin-%s", "linux-%s"];
const PLATFORMS = ["darwin-%s"];
const ARCHITECTURES = ["x64", "arm64"];

for (const platform of PLATFORMS) {
  for (const arch of ARCHITECTURES) {
    copyBinaryToNativePackage(platform, arch);
  }
}

writeManifest();
