use std::fmt;

#[derive(Eq, PartialEq)]
pub enum ProjectType {
    Javascript,
    Rust,
    Go,
    C,
    Cpp,
    Python,
    Java,
    Kotlin,
    Swift,
    Php,
    Ruby,
    Shell,
    Dart,
    Haskell,
    Scala,
    Perl,
    R,
    Elixir,
    CSharp,
    FSharp,
    Lua,
}
impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ProjectType::Javascript => "Javascript",
            ProjectType::Rust => "Rust",
            ProjectType::Go => "Go",
            ProjectType::C => "C",
            ProjectType::Cpp => "C++",
            ProjectType::Python => "Python",
            ProjectType::Java => "Java",
            ProjectType::Kotlin => "Kotlin",
            ProjectType::Swift => "Swift",
            ProjectType::Php => "PHP",
            ProjectType::Ruby => "Ruby",
            ProjectType::Shell => "Shell",
            ProjectType::Dart => "Dart",
            ProjectType::Haskell => "Haskell",
            ProjectType::Scala => "Scala",
            ProjectType::Perl => "Perl",
            ProjectType::R => "R",
            ProjectType::Elixir => "Elixir",
            ProjectType::CSharp => "C#",
            ProjectType::FSharp => "F#",
            ProjectType::Lua => "Lua",
        };
        write!(f, "{}", name)
    }
}
impl ProjectType {
    pub fn from(file: &str) -> Vec<Self> {
        let mut types = Vec::new();

        match file {
            // JavaScript/TypeScript specific files
            "package.json" | "yarn.lock" | "pnpm-lock.yaml" | "vite.config.js"
            | "webpack.config.js" => {
                types.push(Self::Javascript);
            }
            // Rust specific files
            "Cargo.toml" | "Cargo.lock" | "build.rs" => {
                types.push(Self::Rust);
            }
            // Go specific files
            "go.mod" | "go.sum" => {
                types.push(Self::Go);
            }
            // C-specific files
            "Makefile" | "config.h" => {
                types.push(Self::C);
            }
            // C++ specific files
            "CMakeLists.txt" | ".clang-format" | ".clang-tidy" => {
                types.push(Self::Cpp);
                types.push(Self::C);
            }
            // Python specific files
            "requirements.txt" | "Pipfile" | "pyproject.toml" | "setup.py" | "tox.ini" => {
                types.push(Self::Python);
            }
            // Java specific files
            "pom.xml" | "build.gradle" | "settings.gradle" => {
                types.push(Self::Java);
            }
            // Kotlin specific files
            "build.gradle.kts" | "settings.gradle.kts" => {
                types.push(Self::Kotlin);
            }
            // Swift specific files
            "Package.swift" | "Info.plist" | ".xcodeproj" | ".xcworkspace" => {
                types.push(Self::Swift);
            }
            // PHP specific files
            "composer.json" | "composer.lock" => {
                types.push(Self::Php);
            }
            // Ruby specific files
            "Gemfile" | "Gemfile.lock" | "Rakefile" => {
                types.push(Self::Ruby);
            }
            // Shell specific files
            ".sh" | ".bashrc" | ".zshrc" | ".profile" => {
                types.push(Self::Shell);
            }
            // Dart specific files
            "pubspec.yaml" | ".packages" => {
                types.push(Self::Dart);
            }
            // Haskell specific files
            "stack.yaml" | "cabal.project" | ".ghci" => {
                types.push(Self::Haskell);
            }
            // Scala specific files
            "build.sbt" => {
                types.push(Self::Scala);
            }
            // Perl specific files
            "Makefile.PL" => {
                types.push(Self::Perl);
            }
            // R specific files
            "DESCRIPTION" | "NAMESPACE" => {
                types.push(Self::R);
            }
            // Elixir specific files
            "mix.exs" => {
                types.push(Self::Elixir);
            }
            // C# specific files
            ".csproj" | ".sln" | "app.config" => {
                types.push(Self::CSharp);
            }
            // F# specific files
            ".fsproj" => {
                types.push(Self::FSharp);
            }
            // Lua specific files
            "init.lua" | ".luacheckrc" => {
                types.push(Self::Lua);
            }
            _ => {}
        }

        types
    }

    pub fn get_files(&self) -> Vec<&str> {
        match self {
            Self::Javascript => vec![
                ".ts",
                ".js",
                ".jsx",
                ".tsx",
                ".json",
                "package.json",
                "yarn.lock",
                "pnpm-lock.yaml",
                "vite.config.js",
                "webpack.config.js",
                ".eslintrc",
                ".prettierrc",
                ".babelrc",
                ".svelte",
                ".vue",
                ".nuxt",
                ".astro",
            ],
            Self::Rust => vec![
                ".rs",
                "Cargo.toml",
                "build.rs",
                ".rustfmt.toml",
                ".clippy.toml",
            ],
            Self::Go => vec![
                ".go",
                "go.mod",
                "go.sum",
                ".golangci.yaml",
                "Makefile",
                "Dockerfile",
            ],
            Self::C => vec![".c", ".h", ".o", "Makefile", "config.h", "CMakeLists.txt"],
            Self::Cpp => vec![
                ".cpp",
                ".hpp",
                ".hxx",
                ".cxx",
                ".cc",
                ".o",
                "Makefile",
                "CMakeLists.txt",
                ".clang-format",
                ".clang-tidy",
            ],
            Self::Python => vec![
                ".py",              // Python source files
                "requirements.txt", // Dependency management
                "Pipfile",          // Pipenv dependency management
                "pyproject.toml",   // PEP 518 configuration
                "setup.py",         // Package configuration
                ".pylintrc",        // Pylint configuration
                "tox.ini",          // Testing framework configuration
                "Dockerfile",       // Docker build configuration
            ],
            Self::Java => vec![
                ".java",           // Java source files
                "pom.xml",         // Maven project descriptor
                "build.gradle",    // Gradle project descriptor
                "settings.gradle", // Gradle settings
                ".classpath",      // Eclipse project configuration
                ".project",        // Eclipse project configuration
            ],
            Self::Kotlin => vec![
                ".kt",                 // Kotlin source files
                ".kts",                // Kotlin scripts
                "build.gradle.kts",    // Gradle Kotlin DSL
                "settings.gradle.kts", // Gradle settings Kotlin DSL
            ],
            Self::Swift => vec![
                ".swift",        // Swift source files
                "Package.swift", // Swift package manager manifest
                "Info.plist",    // iOS/macOS app metadata
                ".xcodeproj",    // Xcode project file
                ".xcworkspace",  // Xcode workspace
            ],
            Self::Php => vec![
                ".php",          // PHP source files
                "composer.json", // Composer dependency manager
                "composer.lock", // Composer lock file
                ".env",          // Environment configuration
            ],
            Self::Ruby => vec![
                ".rb",          // Ruby source files
                "Gemfile",      // Ruby dependency management
                "Gemfile.lock", // Lock file for dependencies
                "Rakefile",     // Rake build scripts
            ],
            Self::Shell => vec![
                ".sh",      // Shell scripts
                ".bashrc",  // Bash configuration
                ".zshrc",   // Zsh configuration
                ".profile", // User profile configuration
            ],
            Self::Dart => vec![
                ".dart",        // Dart source files
                "pubspec.yaml", // Dart package manager manifest
                ".packages",    // Dart package resolution
            ],
            Self::Haskell => vec![
                ".hs",           // Haskell source files
                "stack.yaml",    // Stack configuration
                "cabal.project", // Cabal configuration
                ".ghci",         // GHC interactive environment
            ],
            Self::Scala => vec![
                ".scala",    // Scala source files
                "build.sbt", // SBT build file
                ".sc",       // Scala scripts
            ],
            Self::Perl => vec![
                ".pl",         // Perl source files
                ".pm",         // Perl modules
                "Makefile.PL", // Perl makefile
            ],
            Self::R => vec![
                ".R",          // R source files
                "DESCRIPTION", // R package description
                "NAMESPACE",   // R package namespace
            ],
            Self::Elixir => vec![
                ".ex",     // Elixir source files
                ".exs",    // Elixir scripts
                "mix.exs", // Mix build configuration
            ],
            Self::CSharp => vec![
                ".cs",        // C# source files
                ".csproj",    // C# project configuration
                ".sln",       // Solution files
                "app.config", // Application configuration
            ],
            Self::FSharp => vec![
                ".fs",     // F# source files
                ".fsproj", // F# project configuration
            ],
            Self::Lua => vec![
                ".lua",        // Lua source files
                "init.lua",    // Lua entry point
                ".luacheckrc", // Lua lint configuration
            ],
        }
    }
}
