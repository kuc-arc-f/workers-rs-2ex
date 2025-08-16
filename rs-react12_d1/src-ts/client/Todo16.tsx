// src/App.tsx

import React, { useState, useEffect } from 'react';
import { Item } from './types';
import Modal from './Todo16/Modal';
import ItemForm from './Todo16/ItemForm';
import { itemsApi } from './Todo16/itemsApi';
import Head from '../components/Head';

// 新規アイテムの初期値
const emptyItem: Item = {
  id: '',
  title: '',
  content: '',
  content_type: '',
  public_type: '',
  food_orange: false,
  food_apple: false,
  food_banana: false,
  food_melon: false,
  food_grape: false,
  category_food: false,
  category_drink: false,
  category_gadget: false,
  category_sport: false,
  category_government: false,
  category_internet: false,
  category_smartphone: false,
  country_jp: '',
  country_en: '',
  prefecture_jp: '',
  prefecture_en: '',
  post_no_jp: '',
  post_no_en: '',
  address_1_jp: '',
  address_1_en: '',
  address_2_jp: '',
  address_2_en: '',
  address_other_jp: '',
  address_other_en: '',
  pub_date1: '',
  pub_date2: '',
  pub_date3: '',
  pub_date4: '',
  pub_date5: '',
  pub_date6: '',
  qty1: '',
  qty2: '',
  qty3: '',
  qty4: '',
  qty5: '',
  qty6: '',
};

// 初期表示用のダミーデータ
const initialItems: Item[] = [
  { ...emptyItem, id: '1', title: '最初の投稿', content: 'これは最初の投稿です。', public_type: 'public', food_apple: true, category_food: true },
  { ...emptyItem, id: '2', title: '二番目の投稿', content: 'これは二番目の投稿です。', public_type: 'private', food_orange: true, category_drink: true },
];

const App: React.FC = () => {
  const [items, setItems] = useState<Item[]>(initialItems);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [editingItem, setEditingItem] = useState<Item | null>(null);

  const fetchItems = async () => {
    try {
      const data = await itemsApi.getAll();
      console.log(data);
      setItems(data.data);
    } catch (err) {
      setError('アイテムの取得に失敗しました');
    } finally {
    }
  };

  useEffect(() => {
    fetchItems();
  }, []);

  // モーダルを開く処理
  const handleOpenModal = (item: Item | null) => {
    // itemがnullの場合は新規作成、そうでない場合は編集
    setEditingItem(item ? { ...item } : { ...emptyItem, id: '' });
    setIsModalOpen(true);
  };

  // モーダルを閉じる処理
  const handleCloseModal = () => {
    setIsModalOpen(false);
    setEditingItem(null);
  };

  // データの保存（新規作成・更新）
  const handleSaveItem = async(itemToSave: Item) => {
    console.log(itemToSave);
    if (itemToSave.id) {
      // IDがあれば更新
      await itemsApi.update(itemToSave.id, itemToSave);
      //setItems(items.map(item => (item.id === itemToSave.id ? itemToSave : item)));
    } else {
      // IDがなければ新規作成
      await itemsApi.create(itemToSave);
    }
    handleCloseModal();
    fetchItems();
  };

  // データの削除
  const handleDeleteItem = async (id: string) => {
    if (window.confirm('本当にこの項目を削除しますか？')) {
      await itemsApi.delete(Number(id));
      fetchItems();
    }
  };

  return (
    <div className="container mx-auto p-4 sm:p-6 lg:p-8">
      <Head />
      <header className="flex justify-between items-center mb-6">
        <h1 className="text-3xl font-bold text-gray-800">CRUD アプリ</h1>
        <button
          onClick={() => handleOpenModal(null)}
          className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 shadow-md transition"
        >
          新規追加
        </button>
      </header>

      {/* アイテム一覧 */}
      <div className="bg-white shadow-md rounded-lg overflow-hidden">
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Title</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Content</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Public</th>
                <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">Actions</th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {items.map(item => (
                <tr key={item.id} className="hover:bg-gray-50">
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">{item.title}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 max-w-xs truncate">{item.content}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{item.public_type}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium space-x-2">
                    <button
                      onClick={() => handleOpenModal(item)}
                      className="text-indigo-600 hover:text-indigo-900"
                    >
                      編集
                    </button>
                    <button
                      onClick={() => handleDeleteItem(item.id)}
                      className="text-red-600 hover:text-red-900"
                    >
                      削除
                    </button>
                  </td>
                </tr>
              ))}
               {items.length === 0 && (
                <tr>
                  <td colSpan={4} className="px-6 py-4 text-center text-gray-500">データがありません。</td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </div>

      {/* 新規作成・編集モーダル */}
      <Modal
        isOpen={isModalOpen}
        onClose={handleCloseModal}
        title={editingItem?.id ? '項目を編集' : '項目を新規追加'}
      >
        {editingItem && (
          <ItemForm
            initialData={editingItem}
            onSave={handleSaveItem}
            onCancel={handleCloseModal}
          />
        )}
      </Modal>
    </div>
  );
};

export default App;