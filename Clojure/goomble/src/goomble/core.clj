(ns goomble.core
  (:gen-class))

(def num-goomblers 12)
(def max-balance 100)
(def num-presses (* num-goomblers max-balance))

(def goomble-balance (ref 0))

(defn init-goomblers
  [num-goomblers]
  (repeatedly
   num-goomblers
   (fn [] (ref {:balance (rand-int max-balance)}))))

(defn lucky
  [goomble-balance goombler]
  (dosync
   (let [balance (:balance @goombler)]
     (when (pos? balance)
       (alter goombler assoc :balance (dec balance))
       (alter goomble-balance inc)))))

(defn run-simulation
  [goomble-balance goomblers num-presses]
  (let [players (repeatedly num-presses #(rand-nth goomblers))]
    (dorun (pmap (partial lucky goomble-balance) players))))

(defn goomblers-total-balance
  [goomblers]
  (reduce (fn [sum gmb] (+ sum (:balance @gmb))) 0 goomblers))

(defn -main
  []
  (let [goomblers (init-goomblers num-goomblers)
        initial-goomblers-total-balance
        (goomblers-total-balance goomblers)]
    (println (str "Initial goomblers total balance is "
                  initial-goomblers-total-balance))
    (run-simulation goomble-balance goomblers num-presses)
    (shutdown-agents)
    (doseq [g goomblers]
      (println (str "Goombler has a final balance of " (:balance @g))))
    (println (str "The total Goomblers balance is "
                  (goomblers-total-balance goomblers)))
    (println (str "The Goomble balance is " @goomble-balance))))