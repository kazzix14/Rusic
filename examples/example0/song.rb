require 'jungru'
require 'jungru/support'
require 'debug'

j = ::Piece.new

#j.instrument :inst1 do |i|
#  i.before_each_note do |i, n|
#    #i.offset -4.0
#  end
#  i.signal do |i, n, l, t|
#    i.out (
#        sin(n[:freq], t) \
#        + \
#        sin(n[:freq] * 1.2 + 0.1 * sin(0.2, t), t) \
#        + \
#        sin(n[:freq] * 1.5 + 0.1 * sin(0.1, t), t) * (0.8 + 0.2 * sin(0.13, t))
#      )
#          .amp(adsr(2.0, 4.0, l, 0.4 , 4.0, t))
#          .amp(0.1)
#  end
#end
#
#j.instrument :inst0 do |i|
#  i.signal do |i, n, l, t|
#    if t < 2 * l
#      i.out Math.sin(n[:f] * 2.0 * Math::PI * t) * n[:v]
#    else
#      i.out nil
#    end
#  end
#end

j.instrument :kick do |i|
  i.init do |i|
  end

  i.before_each_note do |i, n|
    i.save :last, 0.0
    i.save :osc, ::Oscillator.new(0.0)
    i.save :whead, 0

    buf = []

    for idx in 0...44100
      buf.push 0
    end

    i.save :buf, buf
  end

  i.signal do |i, n, l, t, dt|
    if t < l
      
      osc = i.load :osc
      i.out osc.sin(n[:f], dt) * (1.0 - (t / (15.0 *l))) ** 4
      i.save :osc, osc
    end
  end
end

j.meta do |m|
  m.bpm 178.0
  m.composite \
  '
    a
  '
end

j.track :kick , :kick do |t|
  t.symbols do |k, i|
    { f: 440.0.eq12.sample }
  end
  
  t.section :a do |s|
    s.division 1, 8
    s.length 8, 1
    
    str = ""
    
    for _ in 0...32
      str << %w(a b c d e f).sample
    end
    
    s.sheet str
  end
end

j.gen :wav, 'out.wav'