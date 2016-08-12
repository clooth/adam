# Uncomment this line to define a global platform for your project
platform :ios, '10.0'

target 'Adam' do
  use_frameworks!

  pod 'Realm', git: 'https://github.com/realm/realm-cocoa.git', branch: 'master', submodules: true
  pod 'RealmSwift', git: 'https://github.com/realm/realm-cocoa.git', branch: 'master', submodules: true

  pod 'RxSwift', git: 'https://github.com/ReactiveX/RxSwift', branch: 'swift-3.0', submodules: true
  pod 'RxCocoa', git: 'https://github.com/ReactiveX/RxSwift', branch: 'swift-3.0', submodules: true
  pod 'RxBlocking', git: 'https://github.com/ReactiveX/RxSwift', branch: 'swift-3.0', submodules: true
end

target 'AdamTests' do
  use_frameworks!

  pod 'RxTests', git: 'https://github.com/ReactiveX/RxSwift', branch: 'swift-3.0', submodules: true
end
