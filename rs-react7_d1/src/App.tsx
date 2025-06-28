// src/App.js
import React, { useState, useEffect } from 'react';
import ItemList from './components/ItemList';
import ItemModal from './components/ItemModal';
//import { getItems, createItem, updateItem, deleteItem } from './api/mockApi';
import { itemsApi } from './client/api/items';

function App() {
  const [items, setItems] = useState([]);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [currentItem, setCurrentItem] = useState(null);

  // 初回レンダリング時にデータを取得
  useEffect(() => {
    _loadItems();
  }, []);

  const _loadItems = async () => {
//   const fetchedItems = await getItems();
    const fetchedItems = await itemsApi.getAll();
    console.log(fetchedItems);
    setItems(fetchedItems.data);
  };

  const handleOpenModalForCreate = () => {
    setCurrentItem(null); // 新規作成モード
    setIsModalOpen(true);
  };

  const handleOpenModalForEdit = (item) => {
    setCurrentItem(item); // 編集モード
    setIsModalOpen(true);
  };

  const handleCloseModal = () => {
    setIsModalOpen(false);
    setCurrentItem(null);
  };

  const handleSaveItem = async (itemData) => {
    console.log(itemData);
    if (currentItem) {
      // 更新
      //await updateItem(currentItem.id, itemData);
      await itemsApi.update(currentItem.id, itemData);
    } else {
      // 新規作成
      await itemsApi.create(itemData);
    }
    _loadItems(); // データを再読み込み
    handleCloseModal();
  };

  const handleDeleteItem = async (id) => {
    if (window.confirm('本当にこの項目を削除しますか？')) {
      //await deleteItem(id);
      await itemsApi.delete(id);
      _loadItems(); // データを再読み込み
    }
  };

  return (
    <div className="bg-gray-100 min-h-screen">
      <header className="bg-blue-600 text-white p-4 shadow-md">
        <h1 className="text-2xl font-bold text-center">CRUDアプリ</h1>
      </header>

      <main className="container mx-auto p-4 md:p-8">
        <div className="flex justify-end mb-4">
          <button
            onClick={handleOpenModalForCreate}
            className="bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded shadow-lg transition duration-200"
          >
            新規追加
          </button>
        </div>

        <ItemList
          items={items}
          onEdit={handleOpenModalForEdit}
          onDelete={handleDeleteItem}
        />
      </main>

      {isModalOpen && (
        <ItemModal
          item={currentItem}
          onClose={handleCloseModal}
          onSave={handleSaveItem}
        />
      )}
    </div>
  );
}

export default App;