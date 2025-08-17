import React, { useState, useEffect } from 'react';
import {
  format,
  addMonths,
  subMonths,
  startOfMonth,
  endOfMonth,
  startOfWeek,
  endOfWeek,
  eachDayOfInterval,
  isSameMonth,
  isSameDay,
} from 'date-fns';
import PlanDialog from './Plan/PlanDialog';
import Api from "./Plan/api";
import dayjs from "dayjs";
import Head from '../components/Head';
let targetDate = "";

function App() {
  const [currentDate, setCurrentDate] = useState(new Date('2025-07-01')); // 画像に合わせて初期値を設定
  const [plans, setPlans] = useState([
    // ダミーデータ
    { id: 1, date: '2025-07-15', content: 'c1' },
  ]);
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [editingPlan, setEditingPlan] = useState(null);

  // --- カレンダー描画ロジック ---
  const monthStart = startOfMonth(currentDate);
  const monthEnd = endOfMonth(currentDate);
  const startDate = startOfWeek(monthStart);
  const endDate = endOfWeek(monthEnd);
  const days = eachDayOfInterval({ start: startDate, end: endDate });
  const weekdays = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

  const fetchTodos = async (targetDate: string) => {
    try {
      const resp = await Api.list(targetDate);
      setPlans(resp);
    } catch (error) {
      console.error('Error fetching todos:', error);
    }
  };  
  useEffect(() => {
    const now = dayjs();
    targetDate = now.format('YYYY-MM-DD'); 
    setCurrentDate(targetDate)
    console.log(targetDate); // => "2025-08-13"
    fetchTodos(targetDate);
  }, []);

  // --- イベントハンドラ ---
  const handlePrevMonth = () => {
    const now = dayjs(targetDate);
    const startOfLastMonth = now.subtract(1, 'month').startOf('month');
    //console.log(startOfLastMonth.format('YYYY-MM-DD'));
    targetDate = startOfLastMonth.format('YYYY-MM-DD');
    console.log("targetDate=", targetDate);
    fetchTodos(targetDate)
    const target = subMonths(currentDate, 1)
    console.log("pre=", target);
    setCurrentDate(targetDate)
    //setCurrentDate(subMonths(currentDate, 1))
  };
  const handleNextMonth = () => {
    const now = dayjs(targetDate);
    const startOfNextMonth = now.add(1, 'month').startOf('month');
    targetDate = startOfNextMonth.format('YYYY-MM-DD');
    console.log(startOfNextMonth.format('YYYY-MM-DD'));
    console.log("targetDate=", targetDate);
    fetchTodos(targetDate)
    //setCurrentDate(addMonths(currentDate, 1))
    setCurrentDate(targetDate)
  };

  const handleOpenDialog = (plan, day) => {
    console.log(plan);
    // 新規作成の場合、日付をセット
    if (!plan) {
      setEditingPlan({ date: format(day, 'yyyy-MM-dd'), content: '' });
    } else {
      const target = dayjs(plan.date);
      targetDate = target.format('YYYY-MM-DD');
      plan.date = targetDate
      console.log(targetDate);
      setEditingPlan(plan);
    }
    setIsDialogOpen(true);
  };

  const handleCloseDialog = () => {
    setIsDialogOpen(false);
    setEditingPlan(null);
  };

  const handleSavePlan = async(planToSave) => {
    planToSave.p_date = planToSave.date;
    console.log(planToSave);
    if (planToSave.id) {
      // 更新
      await Api.update(planToSave, "")
      //setPlans(plans.map(p => (p.id === planToSave.id ? planToSave : p)));
    } else {
      // 新規作成
      await Api.create(planToSave, "");
    }
    handleCloseDialog();
    fetchTodos(targetDate);
  };

  const handleDelete = async(id) => {
    console.log("id=", id);
    if (window.confirm("delete , OK?")) {
      const item = {id: id}
      await Api.delete(item, "");
      fetchTodos(targetDate);
    }
  };


  return (
    <div className="p-4 sm:p-8 max-w-5xl mx-auto">
      <Head />
      
      {/* --- ヘッダー --- */}
      <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-6">
        <h1 className="text-3xl font-bold text-gray-800 mb-4 sm:mb-0">
          Plan: {format(currentDate, 'yyyy-MM')}
        </h1>
        <div className="flex items-center space-x-2">
          <button
            onClick={() => handleOpenDialog(null, new Date())}
            className="px-4 py-2 bg-blue-600 text-white font-semibold rounded-md shadow-sm hover:bg-blue-700"
          >
            Create
          </button>
          <div className="flex">
             <button onClick={handlePrevMonth} className="px-3 py-2 border border-gray-300 bg-white text-gray-700 rounded-l-md hover:bg-gray-50">Before</button>
             <button onClick={handleNextMonth} className="px-3 py-2 border-t border-b border-r border-gray-300 bg-white text-gray-700 rounded-r-md hover:bg-gray-50">Next</button>
          </div>
        </div>
      </div>

      {/* --- カレンダー --- */}
      <div className="border border-gray-200">
        {/* 曜日ヘッダー */}
        <div className="grid grid-cols-7 bg-gray-50">
          {weekdays.map((day, index) => (
            <div
              key={day}
              className={`text-center font-semibold p-2 border-b border-gray-200
                ${index === 0 ? 'text-red-600' : ''}
                ${index === 6 ? 'text-blue-600' : ''}
              `}
            >
              {day}
            </div>
          ))}
        </div>
        {/* 日付グリッド */}
        <div className="grid grid-cols-7">
          {days.map(day => {
            const isCurrentMonth = isSameMonth(day, monthStart);
            const dailyPlans = plans.filter(p => isSameDay(new Date(p.date), day));
            return (
              <div
                key={day.toString()}
                className={`h-32 p-2 border-r border-b border-gray-200 flex flex-col
                  ${isCurrentMonth ? 'bg-white' : 'bg-gray-50'}
                `}
                onClick={() => handleOpenDialog(null, day)} // 日付セルクリックで新規作成
              >
                <span
                  className={`font-medium
                    ${!isCurrentMonth ? 'text-gray-400' : 'text-gray-800'}
                  `}
                >
                  {format(day, 'd')}
                </span>
                <div className="mt-1 flex-grow overflow-y-auto">
                  {dailyPlans.map(plan => (
                    <div
                      key={plan.id}
                      className="bg-blue-100 border border-blue-200 rounded p-1 text-xs mb-1 break-words"
                      onClick={(e) => e.stopPropagation()} // 親のクリックイベントを抑制
                    >
                      <p className="text-gray-800">{plan.content}</p>
                      <button
                        onClick={() => handleOpenDialog(plan)}
                        className="text-blue-600 hover:underline font-semibold"
                      >
                        Edit
                      </button>

                      <button className="mx-2 mt-2"
                      onClick={() => handleDelete(plan.id)}
                        >[ Delete ]</button>
                    </div>
                  ))}
                </div>
              </div>
            );
          })}
        </div>
      </div>
      
      {/* --- ダイアログ --- */}
      {isDialogOpen && (
        <PlanDialog
          plan={editingPlan}
          onClose={handleCloseDialog}
          onSave={handleSavePlan}
        />
      )}
    </div>
  );
}

export default App;