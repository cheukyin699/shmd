#lang racket/base

(require racket/contract/base
         racket/function
         racket/string
         racket/list
         racket/bool
         (prefix-in mp3: binary-class/mp3))

(require (prefix-in db: "database.rkt"))

(provide
  (contract-out
    [get-audio-files (-> path-string? (listof path?))]
    [get-media-data (-> path-string? (listof db:media?))]))

(define AUDIO-EXTENSIONS
  (list "mp3" "wav" "aiff"))

(define (get-audio-files root-folder)
  ;; We assume that an extension exists; nothing meaningful if there is none
  (define (path->extension p)
    (last (string-split (path->string p) ".")))

  (define (audio-file? p)
    (and (file-exists? p)
         (not (false? (member (path->extension p) AUDIO-EXTENSIONS)))))

  (filter
    audio-file?
    (for/list
      ([f (in-directory root-folder)])
      f)))

(define (get-media-data root-folder)
  ;; path->media: path? -> (or/c db:media? #f)
  (define (path->media p)
    (with-handlers ([exn:fail:contract? (λ (err) (db:media #f "" "" "" (path->string p)))])
      (let* ([i (mp3:read-id3 p)]
             [location (path->string p)]
             [title (mp3:song i)]
             [album (mp3:album i)]
             [artist (mp3:artist i)])
        (db:media #f
                  (or title "")
                  (or artist "")
                  (or album "")
                  location))))

  (filter (negate false?) (map path->media (get-audio-files root-folder))))
