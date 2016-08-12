//
//  ViewController.swift
//  Adam
//
//  Created by Nico Hämäläinen on 12/08/2016.
//  Copyright © 2016 sizeof.io. All rights reserved.
//

import Foundation
import UIKit
import RxSwift
import RxCocoa
import RealmSwift

class Lap: Object {
  dynamic var time: TimeInterval = Date().timeIntervalSinceReferenceDate
}

let formatter: DateFormatter = {
  let f = DateFormatter()
  f.timeStyle = .long
  return f
}()

class ViewController: UITableViewController {
  let bag = DisposeBag()
  let realm = try! Realm()
  
  override func viewDidLoad() {
    super.viewDidLoad()
    
    // Reset since we're using UITableViewController
    tableView.dataSource = nil
    tableView.register(UITableViewCell.self, forCellReuseIdentifier: "Cell")
    
    // Add some buttons
    navigationItem.rightBarButtonItem = UIBarButtonItem(title: "Lap", style: .done, target: nil, action: nil)
    
    // Bind number of laps to the title of the controller
    realm.allObjects(ofType: Lap.self).asObservable()
      .map { "\($0.count) laps" }
      .subscribe(onNext: { [unowned self] text in
        self.title = text
      })
      .addDisposableTo(bag)
    
    // Bind all laps to cells of the table view
    realm.allObjects(ofType: Lap.self).asObservableArray()
      .map { $0.reversed() }
      .bindTo(tableView.rx_items(cellIdentifier: "Cell", cellType: UITableViewCell.self)) { (tableView, element, cell) in
        cell.textLabel?.text = formatter.string(from: Date(timeIntervalSinceReferenceDate: element.time))
      }
      .addDisposableTo(bag)
    
    // Bind right-side button to adding a new lap
    navigationItem.rightBarButtonItem!
      .rx_tap
      .map { Lap() }
      .bindTo(Realm.rx_add())
      .addDisposableTo(bag)
  }

  override func didReceiveMemoryWarning() {
    super.didReceiveMemoryWarning()
    // Dispose of any resources that can be recreated.
  }


}

