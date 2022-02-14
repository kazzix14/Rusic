require 'jungru'

j = ::Jungru::Piece.new

j.instrument :inst0 do |i|
  i.init do |i|
  end
  
  i.before_each_note do |i, n|
  end

  i.signal do |i, n, t|
    f = 440.0 * t * 2 * Math::PI

    if t < 0.5 && n[:tone] == 0
      i.out n[:vel] * Math.sin(f) * (1.0 - 0.8 * t)
    elsif t < 0.5 && n[:tone] == 1
      i.out t * Math.sin(f)
    else
      i.out nil
    end
  end
end

j.meta do |m|
  m.bpm 162.0
  m.composite \
  '
    a b
  '
end

j.track :track1, :inst0 do |t|
  t.symbol :a, {vel: 0.5, tone: 0}
  t.symbol :b, {vel: 0.5, tone: 1}

  t.section :a do |s|
    s.division 1, 16
    s.sheet \
    "
      aaba aaba
    "
  end

  t.section :b do |s|
    s.division 1, 16
    s.sheet \
    "
      bbab bbab
    "
  end
end

j.gen :wav, 'out.wav'