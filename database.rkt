#lang racket/base

(require racket/contract/base
         racket/match
         racket/list
         racket/function
         racket/string
         db
         sql)

(require (prefix-in config: "config.rkt"))

(provide
  (contract-out
    [count-media (->* ()
                      (#:artist string?
                       #:album string?
                       #:keyword string?)
                      (not/c negative?))]
    [get-media (->* ()
                    (#:artist string?
                     #:album string?
                     #:keyword string?
                     #:limit positive?
                     #:offset positive?)
                    (listof media?))])

  make-media-query
  media)

(define dbc (postgresql-connect #:socket 'guess
                                #:user config:db-username
                                #:password config:db-password
                                #:database config:db-database))

(struct media (id title artist album location)
  #:mutable
  #:transparent)

(define (row->media row)
  (match-let ([(vector id title artist album location) row])
    (media id title artist album location)))

(define (string-empty? s)
  (zero? (string-length s)))

(define (make-media-single-condition condition)
  (match-let ([(list col val) condition])
    (scalar-expr-qq
      (like (Ident:AST ,(make-ident-ast col))
            (|| "%" (ScalarExpr:AST ,val) "%")))))

(define (make-media-query artist album keyword)
  (let* ([conditions `((artist ,artist)
                       (album ,album)
                       (title ,keyword))]
         [non-empty-conditions (filter (compose (negate string-empty?) second) conditions)])
    (case (length non-empty-conditions)
      [(0) (scalar-expr-qq (= 1 1))]
      [(1) (make-media-single-condition (first non-empty-conditions))]
      [else
        (scalar-expr-qq
          (ScalarExpr:INJECT
            ,(string-join
               (map
                 (compose sql-ast->string make-media-single-condition)
                 non-empty-conditions)
               " and ")))])))

(define (get-media #:artist [artist ""]
                   #:album [album ""]
                   #:keyword [keyword ""]
                   #:limit [limit 10]
                   #:offset [offset 0])
  (map row->media
       (query-rows
         dbc
         (select id title artist album location
                 #:from media
                 #:where (ScalarExpr:AST ,(make-media-query artist album keyword))))))

(define (count-media #:artist [artist ""]
                     #:album [album ""]
                     #:keyword [keyword ""])
  (query-value
    dbc
    (select (count-all)
            #:from media
            #:where (ScalarExpr:AST ,(make-media-query artist album keyword)))))
