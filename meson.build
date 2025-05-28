project('notify', 'c',
  version : '0.1',
  default_options : ['warning_level=3'])

notify_dep = dependency('libnotify')



executable('notify',
           sources: ['src/main.c'], # Your source file(s)
           dependencies: [notify_dep],
           install: true)

