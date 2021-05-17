pub(crate) fn dirs() -> Vec<&'static str> {
	vec![
		"__tests__",
		"__test__",
		"test",
		"tests",
		"powered-test",
		"docs",
		"doc",
		".idea",
		".vscode",
		"website",
		"images",
		"assets",
		"example",
		"examples",
		"coverage",
		".nyc_output",
		".circleci",
		".github",
		"ci",
	]
}

pub(crate) fn files() -> Vec<&'static str> {
	vec![
		"Jenkinsfile",
		"Makefile",
		"Gulpfile.js",
		"Gruntfile.js",
		"gulpfile.js",
		"eslint",
		"stylelint.config.js",
		"htmllint.js",
		"appveyor.yml",
		"circle.yml",
		"CHANGES",
		"changelog",
		"LICENSE.txt",
		"LICENSE",
		"LICENSE-MIT",
		"LICENSE.BSD",
		"license",
		"LICENCE.txt",
		"LICENCE",
		"LICENCE-MIT",
		"LICENCE.BSD",
		"licence",
		"AUTHORS",
		"CONTRIBUTORS",
		"jest.config.js",
		"karma.conf.js",
		"wallaby.js",
		"wallaby.conf.js",
		"tsconfig.json",
		"tslint.json",
	]
}

pub(crate) fn extensions() -> Vec<&'static str> {
	vec!["markdown", "md", "mkd", "ts", "jst", "coffee", "tgz", "swp"]
}
