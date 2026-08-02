;;; rayan-mode.el --- Major mode for Rayan configuration fields -*- lexical-binding: t -*-

;; Copyright (C) 2026 Rayan Contributors
;; License: MIT

;;; Commentary:
;; Major mode for editing Rayan fields (.rayan) with syntax highlighting and LSP support.

;;; Code:

(require 'rx)
(require 'eglot)

(defconst rayan-mode-syntax-table
  (let ((table (make-syntax-table)))
    (modify-syntax-entry ?# "<" table)
    (modify-syntax-entry ?\n ">" table)
    table))

(defvar rayan-font-lock-keywords
  `((,(rx bol (1+ "#") (1+ space) (group (1+ nonl)))
     (1 font-lock-doc-face))
    (,(rx "package:" (1+ space) (group (1+ word)))
     (1 font-lock-function-name-face))
    (,(rx "service:")
     (0 font-lock-keyword-face))))

;;;###autoload
(define-derived-mode rayan-mode prog-mode "Rayan"
  "Major mode for editing Rayan configuration fields."
  :syntax-table rayan-mode-syntax-table
  (setq font-lock-defaults '(rayan-font-lock-keywords)))

;;;###autoload
(add-to-list 'auto-mode-alist '("\\.rayan\\'" . rayan-mode))

;; Add to Eglot
(with-eval-after-load 'eglot
  (add-to-list 'eglot-server-programs
               '(rayan-mode . ("rayan" "lsp"))))

(provide 'rayan-mode)
;;; rayan-mode.el ends here
