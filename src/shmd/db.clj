(ns shmd.db
  (:require [next.jdbc :as jdbc]
            [honey.sql :as sql]
            [honey.sql.helpers :as h]))

(def db {:dbtype "postgres" :dbname "shmd" :user "shmd-db"})
(def ds (jdbc/get-datasource db))

(defn- media-query-conditions
  [params]

  (->>
    (select-keys params [:title :artist :album])
    (into [])
    (map (fn [[k v]] `(:like ~k (:|| "%" ~v "%"))))
    (cons :and)))

(defn get-media!
  [params]
  (jdbc/execute!
    ds
    (sql/format
      (merge {:select [:*]
              :from [:media]
              :where (media-query-conditions params)}
             (select-keys params [:offset :limit])))))

(defn count-media!
  [params]
  (:count (jdbc/execute-one!
           ds
           (sql/format
             {:select [[:%count.*]]
              :from [:media]
              :where (media-query-conditions params)}))))

(comment
  (jdbc/execute! ds ["select * from media limit 1"])
  (media-query-conditions {:title "UNION" :artist "Ice"})
  (count-media! {:artist "Ice"}))
