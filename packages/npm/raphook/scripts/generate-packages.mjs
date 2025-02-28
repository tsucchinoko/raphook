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
    console.warn(`Skipping package generation for ${buildName}`);
    return false;
  }

  fs.copyFileSync(binarySource, binaryTarget);
  fs.chmodSync(binaryTarget, 0o755);
  return true;
}

function writeManifest() {
  const manifestPath = resolve(PACKAGES_ROOT, "npm", "raphook", "package.json");

  const manifestData = JSON.parse(
    fs.readFileSync(manifestPath).toString("utf-8")
  );

  // 実際に生成されたパッケージのみを依存関係に追加
  const generatedPackages = [];

  for (const platform of PLATFORMS) {
    for (const arch of ARCHITECTURES) {
      const packageName = getName(platform, arch);
      const packageRoot = resolve(PACKAGES_ROOT, "npm", packageName);
      const binDir = resolve(packageRoot, "bin");
      const os = platform.split("-")[0];
      const ext = os === "win32" ? ".exe" : "";
      const binaryPath = resolve(binDir, `raphook${ext}`);

      if (fs.existsSync(binaryPath)) {
        generatedPackages.push([packageName, rootManifest.version]);
      }
    }
  }

  manifestData.version = rootManifest.version;
  manifestData.optionalDependencies = Object.fromEntries(generatedPackages);

  // 実際に生成されたパッケージのOSとCPUのみを含める
  const generatedOS = [
    ...new Set(generatedPackages.map(([name]) => name.split("-")[1])),
  ];
  manifestData.os = generatedOS;
  manifestData.cpu = ARCHITECTURES;

  console.log("Generated packages:", generatedPackages);
  console.log("Supported OS:", generatedOS);

  console.log(`Update manifest ${manifestPath}`);
  const content = JSON.stringify(manifestData, null, 2);
  fs.writeFileSync(manifestPath, content);
}

// すべてのプラットフォームを対象にするが、実際にバイナリが存在するものだけをパッケージ化
// const PLATFORMS = ["win32-%s", "darwin-%s", "linux-%s"];
const PLATFORMS = ["darwin-%s"];
const ARCHITECTURES = ["x64", "arm64"];

// 各プラットフォーム向けのパッケージを生成
for (const platform of PLATFORMS) {
  for (const arch of ARCHITECTURES) {
    copyBinaryToNativePackage(platform, arch);
  }
}

writeManifest();
